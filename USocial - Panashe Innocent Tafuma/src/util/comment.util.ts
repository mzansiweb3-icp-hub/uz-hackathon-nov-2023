import { CreateCommentDto } from "../modules";
import validateUUIDv4 from "./canister.util";

abstract class CommentConstants {
  public static readonly MAX_COMMENT_LENGTH = 400 as const;
  public static readonly MIN_COMMENT_LENGTH = 1 as const;
}

export function validateCommentContent(
  content: unknown
): asserts content is string {
  if (typeof content !== "string") {
    throw new Error(`Invalid comment content: ${content}`);
  }

  if (
    content.length > CommentConstants.MAX_COMMENT_LENGTH ||
    content.length < CommentConstants.MIN_COMMENT_LENGTH
  ) {
    throw new Error(
      `Comment content must be within ${CommentConstants.MIN_COMMENT_LENGTH} - ${CommentConstants.MAX_COMMENT_LENGTH} characters long`
    );
  }
}

export function sanitiseCreateCommentDto(model: CreateCommentDto): void {
  if (!model.content) {
    throw new Error("Missing comment content");
  }

  validateCommentContent(model.content);

  if (!model.postId || !model.userId) {
    throw new Error("Missing post or user id");
  }

  validateUUIDv4(model.postId);
  validateUUIDv4(model.userId);
}
