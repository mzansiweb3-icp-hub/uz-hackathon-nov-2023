import {
  CreateMessageDto,
  Message,
  PartialMessage,
  UpdateMessageDto,
} from "../modules";
import validateUUID from "./canister.util";

abstract class MessageContent {
  public static readonly MAX_CONTENT_LENGTH = 800 as const;
  public static readonly MIN_CONTENT_LENGTH = 1 as const;
}

export function validateContent(content: string): asserts content is string {
  if (typeof content !== "string") {
    throw new Error(`Invalid content: ${content}`);
  }

  if (
    content.length > MessageContent.MAX_CONTENT_LENGTH ||
    content.length < MessageContent.MIN_CONTENT_LENGTH
  ) {
    throw new Error(
      `Content must be between ${MessageContent.MIN_CONTENT_LENGTH} and ${MessageContent.MAX_CONTENT_LENGTH} characters long`
    );
  }
}

export default function sanitiseCreateMessageDto(
  model: CreateMessageDto
): asserts model is CreateMessageDto {
  validateContent(model.content);
  validateUUID(model.conversationId, model.userId);
}

export function sanitiseUpdateMessageDto(
  message: Message,
  update: UpdateMessageDto
): PartialMessage {
  let content = message.content;
  if (update?.content && typeof update.content === "string") {
    validateContent(update.content);
    content = update.content;
  }

  return {
    content,
  };
}
