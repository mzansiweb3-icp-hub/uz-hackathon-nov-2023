import { Principal, Record, Vec, text } from "azle";
import { nat64 } from "azle";

export abstract class CommentConstants {
  public static readonly MAX_COMMENT_LENGTH = 400 as const;
  public static readonly MIN_COMMENT_LENGTH = 1 as const;
}

export const Comment = Record({
  postId: text,
  content: text,
  id: text,
  createdAt: nat64,
  userId: text,
  createdBy: Principal,
  likes: Vec(text), //an array of user ids
});

export type Comment = typeof Comment;

export const CreateCommentDto = Record({
  userId: text,
  postId: text,
  content: text,
});

export type CreateCommentDto = typeof CreateCommentDto;
