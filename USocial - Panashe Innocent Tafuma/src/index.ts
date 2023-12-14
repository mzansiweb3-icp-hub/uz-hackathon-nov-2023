import {
  StableBTreeMap,
  Vec,
  Result,
  nat64,
  ic,
  Opt,
  text,
  Canister,
  query,
  update,
  Ok,
  Err,
  int8,
  TypeMapping,
} from "azle";
import { v4 as uuidv4 } from "uuid";
import {
  AcceptFriendRequestDto,
  Comment,
  Conversation,
  CreateCommentDto,
  CreateConversationDto,
  CreateFriendDto,
  CreateMessageDto,
  CreatePostDto,
  CreateUserDto,
  Friend,
  Message,
  Post,
  UpdateMessageDto,
  UpdatePostDto,
  UpdateUserDto,
  User,
} from "./modules";
import {
  InternalCanisterError,
  NotFoundError,
  RequestError,
  UnauthorizedException,
} from "./exceptions";
import getErrorMessage from "./util/exceptions.util";
import {
  sanitiseAcceptFriendRequestDto,
  sanitiseCreateFriendDto,
  sanitiseUserUpdate,
  validateCreateUserDto,
  validateEmail,
} from "./util";
import { sanitisePostUpdate } from "./util/post.util";
import { sanitiseCreateCommentDto } from "./util/comment.util";
import validateUUID, { validateUnixDate } from "./util/canister.util";
import sanitiseCreateConversationDto from "./util/conversation.util";
import sanitiseCreateMessageDto, {
  sanitiseUpdateMessageDto,
} from "./util/message.util";

const USER_STORAGE = StableBTreeMap(text, User, 0);
const POST_STORAGE = StableBTreeMap(text, Comment, 1);
const FRIEND_STORAGE = StableBTreeMap(text, Friend, 2);
const COMMENT_STORAGE = StableBTreeMap(text, Comment, 3);
const CONVERSATION_STORAGE = StableBTreeMap(text, Conversation, 4);
const MESSAGE_STORAGE = StableBTreeMap(text, Message, 5);

function retrieveUsers(): Array<User> {
  return USER_STORAGE.values() as Array<User>;
}

function retrievePosts(): Array<Post> {
  return POST_STORAGE.values() as Array<Post>;
}

function retrieveFriends(): Array<Friend> {
  return FRIEND_STORAGE.values() as Array<Friend>;
}

function retrieveComments(): Array<Comment> {
  return COMMENT_STORAGE.values() as Array<Comment>;
}

function retrieveConversations(): Array<Conversation> {
  return CONVERSATION_STORAGE.values() as Array<Conversation>;
}

function retrieveMessages(): Array<Message> {
  return MESSAGE_STORAGE.values() as Array<Message>;
}

function retrieveUserByIDOrFail(id: string): User {
  const user = USER_STORAGE.get(id) as Opt<TypeMapping<User>>;

  if (!user?.Some) {
    throw new NotFoundError(`User with id = ${id} not found`);
  }

  return user.Some;
}

function retrievePostByIDOrFail(id: string): Post {
  const post = POST_STORAGE.get(id) as Opt<TypeMapping<Post>>;

  if (!post?.Some) {
    throw new NotFoundError(`Post with id = ${id} not found`);
  }

  return post.Some;
}

function retrieveFriendByIDOrFail(id: string): Friend {
  const friend = FRIEND_STORAGE.get(id) as Opt<TypeMapping<Friend>>;

  if (!friend?.Some) {
    throw new NotFoundError(`Friendship entity with id = ${id} not found`);
  }

  return friend.Some;
}

function retrieveCommentByIDOrFail(id: string): Comment {
  const comment = COMMENT_STORAGE.get(id) as Opt<TypeMapping<Comment>>;

  if (!comment?.Some) {
    throw new NotFoundError(`Comment entity with id = ${id} not found`);
  }

  return comment.Some;
}

function retrieveConverationByIDOrFail(id: string): Conversation {
  const conversation = CONVERSATION_STORAGE.get(id) as Opt<
    TypeMapping<Conversation>
  >;

  if (!conversation.Some) {
    throw new NotFoundError(`Converation entity with id = ${id} not found`);
  }

  return conversation.Some;
}

function retrieveMessageByIDOrFail(id: string): Message {
  const message = MESSAGE_STORAGE.get(id) as Opt<TypeMapping<Message>>;

  if (!message.Some) {
    throw new NotFoundError(`Message entity with id = ${id} not found`);
  }

  return message.Some;
}

type StorageType = User | Post | Friend | Comment | Conversation | Message;
type StorageQueryType =
  | "user"
  | "post"
  | "friend"
  | "comment"
  | "converation"
  | "message";

function generateUniqueID(storageType: StorageQueryType): string {
  let STORAGE: Array<StorageType>;

  switch (storageType) {
    case "user":
      STORAGE = retrieveUsers();
      break;
    case "post":
      STORAGE = retrievePosts();
      break;
    case "friend":
      STORAGE = retrieveFriends();
      break;
    case "comment":
      STORAGE = retrieveComments();
      break;
    case "converation":
      STORAGE = retrieveConversations();
      break;
    case "message":
      STORAGE = retrieveMessages();
      break;
  }

  if (!STORAGE) {
    throw new InternalCanisterError(`Storage Unavialable`);
  }

  if (!STORAGE.length) {
    return uuidv4();
  }

  let id: string;

  do {
    id = uuidv4();
  } while (Boolean(STORAGE.find((user) => user.id === id)) === true);

  return id;
}

function assertUserIsUnique(model: CreateUserDto): void {
  const users = retrieveUsers();
  const isUser = users.find(
    (user) =>
      user.email === model.email ||
      user.mobile === model.mobile ||
      user.createdBy.toString() === ic.caller().toString()
  );

  if (isUser) {
    throw new Error("User already exists");
  }
}

export default Canister({
  //USERS
  createUser: update(
    [CreateUserDto],
    Result(User, RequestError),
    (dto: CreateUserDto) => {
      try {
        validateCreateUserDto(dto);

        assertUserIsUnique(dto);

        const user: User = {
          ...dto,
          id: generateUniqueID("user"),
          createdAt: ic.time(),
          createdBy: ic.caller(),
          isVerfied: false,
        };

        USER_STORAGE.insert(user.id, user);

        return Ok(user);
      } catch (error: any) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  deleteUser: update([text], Result(text, RequestError), (id: string) => {
    try {
      validateUUID(id);
      const user = retrieveUserByIDOrFail(id);

      if (user.createdBy.toString() !== ic.caller().toString()) {
        throw new UnauthorizedException();
      }

      retrievePosts()
        .filter(
          (post) =>
            post.userId === id &&
            post.createdBy.toString() == ic.caller().toString()
        )
        .forEach((post) => {
          POST_STORAGE.remove(post.id);
        });

      retrieveComments()
        .filter(
          (comment) =>
            comment.userId === id &&
            comment.createdBy.toString() == ic.caller().toString()
        )
        .forEach((comment) => {
          COMMENT_STORAGE.remove(comment.id);
        });

      retrieveFriends()
        .filter((friend) => friend.sourceId === id || friend.targetId === id)
        .forEach((friend) => {
          FRIEND_STORAGE.remove(friend.id);
        });

      USER_STORAGE.remove(id);

      return Ok(`User with id: ${id} successfully deleted`);
    } catch (error) {
      return Err({
        errMessage: getErrorMessage(error),
      });
    }
  }),
  getAllUsers: query([], Result(Vec(User), RequestError), () => {
    try {
      return Ok(retrieveUsers());
    } catch (error) {
      return Err({
        errMessage: getErrorMessage(error),
      });
    }
  }),
  findUserByID: query([text], Result(User, RequestError), (id: string) => {
    try {
      return Ok(retrieveUserByIDOrFail(id));
    } catch (error) {
      return Err({
        errMessage: getErrorMessage(error),
      });
    }
  }),
  findUserByEmail: query(
    [text],
    Result(User, RequestError),
    (email: string) => {
      try {
        validateEmail(email);

        const user = retrieveUsers().find((user) => user.email === email);

        if (!user) {
          throw new Error(`User with email = ${email} not found`);
        }

        return Ok(User);
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  findUserByMobile: query(
    [text],
    Result(User, RequestError),
    (mobile: string) => {
      try {
        const user = retrieveUsers().find((user) => user.mobile === mobile);

        if (!user) {
          throw new Error(`User with mobile = ${mobile} not found`);
        }

        return Ok(User);
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  getPaginatedUsers: query(
    [int8, int8],
    Result(Vec(User), RequestError),
    (offset: number, limit: number) => {
      try {
        if (typeof offset !== "number" || offset < 0) {
          throw new Error(`Invalid offset number: ${offset}`);
        }

        if (typeof limit !== "number" || limit < 0) {
          throw new Error(`Invalid limit number: ${limit}`);
        }

        return Ok(retrieveUsers().slice(offset, offset + limit));
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  updateUser: update(
    [text, UpdateUserDto],
    Result(User, RequestError),
    (id: string, model: UpdateUserDto) => {
      try {
        validateUUID(id);
        const user = retrieveUserByIDOrFail(id);

        if (user.createdBy.toString() !== ic.caller().toString()) {
          throw new UnauthorizedException();
        }

        const updatedUser: User = {
          ...user,
          ...sanitiseUserUpdate(user, model),
        };

        USER_STORAGE.insert(id, updatedUser);

        return Ok(updatedUser as User);
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  //POSTS
  createPost: update(
    [CreatePostDto],
    Result(Post, RequestError),
    (model: CreatePostDto) => {
      try {
        const user = retrieveUserByIDOrFail(model.userId);

        if (user.createdBy.toString() === ic.caller().toString()) {
          throw new UnauthorizedException();
        }

        const post: Post = {
          id: generateUniqueID("post"),
          createdAt: ic.time(),
          createdBy: ic.caller(),
          userId: model.userId,
          caption: model.caption,
          tags: model.tags,
          likes: [],
        };

        POST_STORAGE.insert(post.id, post);

        return Ok(post);
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  deletePost: update([text], Result(text, RequestError), (id: string) => {
    try {
      validateUUID(id);
      const post = retrievePostByIDOrFail(id);

      if (post.createdBy.toString() === ic.caller().toString()) {
        throw new Error("Unathorized");
      }

      POST_STORAGE.remove(id);

      return Ok(`Post with id = ${id} successfully removed`);
    } catch (error) {
      return Err({
        errMessage: getErrorMessage(error),
      });
    }
  }),
  findPostByID: query([text], Result(Post, RequestError), (id: string) => {
    try {
      validateUUID(id);
      return Ok(retrievePostByIDOrFail(id));
    } catch (error) {
      return Err({
        errMessage: getErrorMessage(error),
      });
    }
  }),
  getAllPosts: query([], Result(Vec(Post), RequestError), () => {
    try {
      return Ok(retrievePosts());
    } catch (error) {
      return Err({
        errMessage: getErrorMessage(error),
      });
    }
  }),
  getAllPostsWithinDateRange: query(
    [nat64, nat64],
    Result(Vec(Post), RequestError),
    (minDate: bigint, maxDate: bigint) => {
      try {
        if (minDate > maxDate) {
          [minDate, maxDate] = [maxDate, minDate];
        }

        const posts = retrievePosts().filter(
          (post) => post.createdAt >= minDate && post.createdAt <= maxDate
        );

        if (!posts.length) {
          throw new Error(
            "No posts were retrieved within the specified date range"
          );
        }

        return Ok(posts);
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  updatePost: update(
    [text, UpdatePostDto],
    Result(Post, RequestError),
    (id: string, model: UpdatePostDto) => {
      try {
        validateUUID(id);

        const post = retrievePostByIDOrFail(id);

        if (post.createdBy.toString() === ic.caller().toString()) {
          throw new UnauthorizedException();
        }

        const updatedPost: Post = {
          ...post,
          ...sanitisePostUpdate(post, model),
        };

        POST_STORAGE.insert(id, updatedPost);

        return Ok(updatedPost);
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  likePost: update(
    [text, text],
    Result(text, RequestError),
    (userId: string, postId: string) => {
      try {
        validateUUID(userId, postId);

        const [post, user] = [
          retrievePostByIDOrFail(postId),
          retrieveUserByIDOrFail(userId),
        ];

        if (user.createdBy.toString() === ic.caller().toString()) {
          throw new UnauthorizedException();
        }

        const hasLiked = Boolean(post.likes.find((id) => id === userId));

        if (hasLiked) {
          return Ok("Aborting operation, you have already liked this post");
        }

        const updatedPost: Post = {
          ...post,
          likes: [...post.likes].concat(userId),
        };

        POST_STORAGE.insert(postId, updatedPost);

        return Ok("Post has been liked successfully");
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  unlikePost: update(
    [text, text],
    Result(text, RequestError),
    (userId: string, postId: string) => {
      try {
        validateUUID(userId, postId);

        const [post, user] = [
          retrievePostByIDOrFail(postId),
          retrieveUserByIDOrFail(userId),
        ];

        if (user.createdBy.toString() === ic.caller().toString()) {
          throw new UnauthorizedException();
        }

        if (!post.likes.find((id) => id === userId)) {
          throw new Error("Aborting operation, you have not liked this post");
        }

        const updatedPost: Post = {
          ...post,
          likes: [...post.likes].filter((id) => id !== userId),
        };

        POST_STORAGE.insert(postId, updatedPost);

        return Ok("Post has been unliked successfully");
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  //FRIENDS
  createFriendshipRequest: update(
    [CreateFriendDto],
    Result(Friend, RequestError),
    (model: CreateFriendDto) => {
      try {
        sanitiseCreateFriendDto(model);

        const [sourceUser, targetUser] = [
          retrieveUserByIDOrFail(model.sourceId),
          retrieveUserByIDOrFail(model.targetId),
        ];

        if (sourceUser.createdBy.toString() === ic.caller().toString()) {
          throw new UnauthorizedException();
        }

        const isFriendshipRequest = retrieveFriends().find(
          (friend) =>
            friend.sourceId === sourceUser.id &&
            friend.targetId === targetUser.id
        );

        if (isFriendshipRequest) {
          throw new Error("Friendship request already exists");
        }

        const friend: Friend = {
          sourceId: sourceUser.id,
          targetId: targetUser.id,
          isAccepted: false,
          id: generateUniqueID("friend"),
          createdAt: ic.time(),
          createdBy: ic.caller(),
        };

        FRIEND_STORAGE.insert(friend.id, friend);

        return Ok(friend);
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  acceptFriendRequest: update(
    [AcceptFriendRequestDto],
    Result(Friend, RequestError),
    (model: AcceptFriendRequestDto) => {
      try {
        sanitiseAcceptFriendRequestDto(model);

        const friend = retrieveFriendByIDOrFail(model.id);
        const targetUser = retrieveUserByIDOrFail(friend.targetId);

        if (targetUser.createdBy.toString() === ic.caller().toString()) {
          throw new UnauthorizedException();
        }

        const updatedFriend: Friend = {
          ...friend,
          isAccepted: model.isAccepted,
        };

        FRIEND_STORAGE.insert(updatedFriend.id, updatedFriend);

        return Ok(updatedFriend);
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  deleteFriendship: update(
    [text, text],
    Result(text, RequestError),
    (id: string, userId: string) => {
      try {
        validateUUID(id, userId);

        const friend = retrieveFriendByIDOrFail(id);

        const user = retrieveUserByIDOrFail(userId);

        if (friend.sourceId !== userId || friend.targetId !== userId) {
          throw new UnauthorizedException();
        }

        if (
          user.createdBy.toString() === ic.caller().toString() ||
          userId !== user.id
        ) {
          throw new UnauthorizedException();
        }

        FRIEND_STORAGE.remove(id);

        return Ok("Friendship has been removed successfully");
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  getAllFriendships: query([], Result(Vec(Friend), RequestError), () => {
    try {
      return Ok(retrieveFriends());
    } catch (error) {
      return Err({
        errMessage: getErrorMessage(error),
      });
    }
  }),
  findFriendById: query([text], Result(Friend, RequestError), (id: string) => {
    try {
      return Ok(retrieveFriendByIDOrFail(id));
    } catch (error) {
      return Err({
        errMessage: getErrorMessage(error),
      });
    }
  }),
  findFriendsByUserId: query(
    [text],
    Result(Vec(Friend), RequestError),
    (userId: string) => {
      try {
        validateUUID(userId);

        const friends = retrieveFriends().filter(
          (friend) => friend.sourceId === userId || friend.targetId === userId
        );

        return Ok(friends);
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  //COMMENTS
  createComment: update(
    [CreateCommentDto],
    Result(Comment, RequestError),
    (model: CreateCommentDto) => {
      try {
        sanitiseCreateCommentDto(model);

        const [post, user] = [
          retrievePostByIDOrFail(model.postId),
          retrieveUserByIDOrFail(model.userId),
        ];

        if (user.createdBy.toString() !== ic.caller.toString()) {
          throw new UnauthorizedException();
        }

        const comment: Comment = {
          id: generateUniqueID("comment"),
          createdAt: ic.time(),
          createdBy: ic.caller(),
          userId: user.id,
          postId: post.id,
          content: model.content,
          likes: [],
        };

        COMMENT_STORAGE.insert(comment.id, comment);

        return Ok(Comment);
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  deleteComment: update(
    [text, text],
    Result(text, RequestError),
    (commentId: string, userId: string) => {
      try {
        validateUUID(commentId, userId);

        const [user, comment] = [
          retrieveUserByIDOrFail(userId),
          retrieveCommentByIDOrFail(commentId),
        ];

        if (user.createdAt.toString() !== ic.caller.toString()) {
          throw new UnauthorizedException();
        }

        if (comment.createdBy.toString() !== ic.caller.toString()) {
          throw new UnauthorizedException();
        }

        COMMENT_STORAGE.remove(comment.id);

        return Ok(`Comment with id = ${comment.id} removed successfully`);
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  findAllCommentsByPostID: query(
    [text],
    Result(Vec(Comment), RequestError),
    (postId: string) => {
      try {
        validateUUID(postId);
        const post = retrievePostByIDOrFail(postId);

        return Ok(
          retrieveComments().filter((comment) => comment.postId === post.id)
        );
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  findAllComments: query([], Result(Vec(Comment), RequestError), () => {
    try {
      return Ok(retrieveComments());
    } catch (error) {
      return Err({
        errMessage: getErrorMessage(error),
      });
    }
  }),
  findAllCommentsByMinDate: query(
    [nat64],
    Result(Vec(Comment), RequestError),
    (minUnixTime: bigint) => {
      try {
        validateUnixDate(minUnixTime);

        return Ok(
          retrieveComments().filter(
            (comment) => comment.createdAt >= minUnixTime
          )
        );
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  getPaginatedCommentsByPostId: query(
    [text, int8, int8],
    Result(Vec(Comment), RequestError),
    (postId: string, offset: number, limit: number) => {
      try {
        validateUUID(postId);

        if (typeof offset !== "number" || offset < 0) {
          throw new Error(`Invalid offset number: ${offset}`);
        }

        if (typeof limit !== "number" || limit < 0) {
          throw new Error(`Invalid limit number: ${limit}`);
        }

        const post = retrievePostByIDOrFail(postId);

        return Ok(
          retrieveComments()
            .filter((comment) => comment.postId === post.id)
            .slice(offset, offset + limit)
        );
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  likeComment: update(
    [text, text],
    Result(text, RequestError),
    (userId: string, commentId: string) => {
      try {
        validateUUID(userId, commentId);

        const [comment, user] = [
          retrieveCommentByIDOrFail(commentId),
          retrieveUserByIDOrFail(userId),
        ];

        if (user.createdBy.toString() === ic.caller().toString()) {
          throw new UnauthorizedException();
        }

        const hasLiked = Boolean(comment.likes.find((id) => id === userId));

        if (hasLiked) {
          return Ok("Aborting operation, you have already liked this comment");
        }

        const updatedComment: Comment = {
          ...comment,
          likes: [...comment.likes].concat(userId),
        };

        POST_STORAGE.insert(commentId, updatedComment);

        return Ok("Comment has been liked successfully");
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  unlikeComment: update(
    [text, text],
    Result(text, RequestError),
    (userId: string, commentId: string) => {
      try {
        validateUUID(userId, commentId);

        const [comment, user] = [
          retrieveCommentByIDOrFail(commentId),
          retrieveUserByIDOrFail(userId),
        ];

        if (user.createdBy.toString() === ic.caller().toString()) {
          throw new UnauthorizedException();
        }

        if (!comment.likes.find((id) => id === userId)) {
          throw new Error(
            "Aborting operation, you have not liked this comment"
          );
        }

        const updatedComment: Comment = {
          ...comment,
          likes: [...comment.likes].filter((id) => id !== userId),
        };

        POST_STORAGE.insert(commentId, updatedComment);

        return Ok("Comment has been unliked successfully");
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  //CONVERSATIONS
  findConversationById: query(
    [text, text],
    Result(Conversation, RequestError),
    (id: string, userId: string) => {
      try {
        validateUUID(id, userId);

        const conversation = retrieveConverationByIDOrFail(id);
        const user = retrieveUserByIDOrFail(userId);

        if (user.createdBy.toString() !== ic.caller().toString()) {
          throw new UnauthorizedException();
        }

        if (
          conversation.sourceId !== user.id ||
          conversation.targetId !== user.id
        ) {
          throw new UnauthorizedException();
        }

        return Ok(conversation);
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  findAllConversationsByUserId: query(
    [text],
    Result(Vec(Conversation), RequestError),
    (userId: string) => {
      try {
        const user = retrieveUserByIDOrFail(userId);

        if (ic.caller().toString() !== user.createdBy.toString()) {
          throw new UnauthorizedException();
        }

        const converations = retrieveConversations().filter(
          (conversation) =>
            conversation.sourceId === user.id ||
            conversation.targetId === user.id
        );

        return Ok(converations);
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  createConversation: update(
    [CreateConversationDto],
    Result(Conversation, RequestError),
    (model: CreateConversationDto) => {
      try {
        sanitiseCreateConversationDto(model);

        const [sourceUser, targetUser] = [
          retrieveUserByIDOrFail(model.sourceId),
          retrieveUserByIDOrFail(model.targetId),
        ];

        if (sourceUser.createdAt.toString() !== ic.caller.toString()) {
          throw new UnauthorizedException();
        }

        const conversation: Conversation = {
          id: generateUniqueID("converation"),
          createdAt: ic.time(),
          createdBy: ic.caller(),
          sourceId: sourceUser.id,
          targetId: targetUser.id,
        };

        CONVERSATION_STORAGE.insert(conversation.id, conversation);

        return Ok(conversation);
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  //MESSAGES
  sendMessage: update(
    [CreateMessageDto],
    Result(Message, RequestError),
    (model: CreateMessageDto) => {
      try {
        sanitiseCreateMessageDto(model);

        const [conversation, user] = [
          retrieveConverationByIDOrFail(model.conversationId),
          retrieveUserByIDOrFail(model.userId),
        ];

        if (user.createdBy.toString() !== ic.caller.toString()) {
          throw new Error("Unuathorized");
        }

        if (
          conversation.sourceId !== user.id ||
          conversation.targetId !== user.id
        ) {
          throw new Error("Unathorized");
        }

        const message: Message = {
          id: generateUniqueID("message"),
          createdAt: ic.time(),
          createdBy: ic.caller(),
          content: model.content,
          userId: model.userId,
          conversationId: model.conversationId,
        };

        MESSAGE_STORAGE.insert(message.id, message);

        return Ok(message);
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  deleteMessage: update(
    [text, text],
    Result(text, RequestError),
    (userId: string, messageId: string) => {
      try {
        validateUUID(userId, messageId);

        const [user, message] = [
          retrieveUserByIDOrFail(userId),
          retrieveMessageByIDOrFail(messageId),
        ];

        if (user.createdAt.toString() !== ic.caller.toString()) {
          throw new UnauthorizedException();
        }

        if (message.userId !== user.id) {
          throw new UnauthorizedException();
        }

        MESSAGE_STORAGE.remove(message.id);

        return Ok("Message deleted successfully");
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  findMessagesByConversationId: query(
    [text, text],
    Result(Vec(Message), RequestError),
    (conversationId: string, userId: string) => {
      try {
        validateUUID(conversationId, userId);

        const [conversation, user] = [
          retrieveConverationByIDOrFail(conversationId),
          retrieveUserByIDOrFail(userId),
        ];

        if (user.createdAt.toString() !== ic.caller().toString()) {
          throw new UnauthorizedException();
        }

        if (
          conversation.sourceId !== user.id ||
          conversation.targetId !== user.id
        ) {
          throw new UnauthorizedException();
        }

        const messages =
          retrieveMessages().filter(
            (message) => message.conversationId === conversation.id
          ) ?? [];

        return Ok(messages);
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  findMessageById: query(
    [text, text],
    Result(Message, RequestError),
    (messageId: string, userId: string) => {
      try {
        validateUUID(messageId, userId);

        const [message, user] = [
          retrieveMessageByIDOrFail(messageId),
          retrieveUserByIDOrFail(userId),
        ];

        if (user.createdBy.toString() !== ic.caller().toString()) {
          throw new UnauthorizedException();
        }

        const conversation = retrieveConverationByIDOrFail(
          message.conversationId
        );

        if (
          conversation.sourceId !== user.id ||
          conversation.targetId !== user.id
        ) {
          throw new UnauthorizedException();
        }

        return Ok(Message);
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
  updateMessage: update(
    [text, UpdateMessageDto],
    Result(Message, RequestError),
    (userId: string, model: UpdateMessageDto) => {
      try {
        validateUUID(userId);

        if (!model.messageId || typeof model.messageId !== "string") {
          throw new Error("Invalid message update payload");
        }

        const [user, message] = [
          retrieveUserByIDOrFail(userId),
          retrieveMessageByIDOrFail(model.messageId),
        ];

        if (user.createdBy.toString() !== ic.caller.toString()) {
          throw new UnauthorizedException();
        }

        if (
          message.createdBy.toString() !== ic.caller().toString() ||
          message.userId !== user.id
        ) {
          throw new UnauthorizedException();
        }

        const updatedMessage: Message = {
          ...message,
          ...sanitiseUpdateMessageDto(message, model),
        };

        MESSAGE_STORAGE.insert(updatedMessage.id, updatedMessage);

        return Ok(updatedMessage);
      } catch (error) {
        return Err({
          errMessage: getErrorMessage(error),
        });
      }
    }
  ),
});

// UUID workaround
globalThis.crypto = {
  // @ts-ignore
  getRandomValues: () => {
    let array = new Uint8Array(32);

    for (let i = 0; i < array.length; i++) {
      array[i] = Math.floor(Math.random() * 256);
    }

    return array;
  },
};
