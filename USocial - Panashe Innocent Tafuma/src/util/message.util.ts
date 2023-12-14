import {
  CreateMessageDto,
  Message,
  MessageConstants,
  PartialMessage,
  UpdateMessageDto,
} from "../modules";
import validateUUID from "./canister.util";

export function validateContent(content: string): asserts content is string {
  if (typeof content !== "string") {
    throw new Error(`Invalid content: ${content}`);
  }

  if (
    content.length > MessageConstants.MAX_CONTENT_LENGTH ||
    content.length < MessageConstants.MIN_CONTENT_LENGTH
  ) {
    throw new Error(
      `Content must be between ${MessageConstants.MIN_CONTENT_LENGTH} and ${MessageConstants.MAX_CONTENT_LENGTH} characters long`
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

export function assertUserCanDeleteMessage(message: Message): void {
  const createdAtInMilliseconds = Number(message?.createdAt);

  if (!createdAtInMilliseconds) {
    return;
  }

  const currentTimeInMilliseconds = new Date().getTime();

  if (
    createdAtInMilliseconds + MessageConstants.MAX_DELETE_MESSAGE_TOLERANCE <
    currentTimeInMilliseconds
  ) {
    throw new Error("Message can no longer be deleted");
  }
}
