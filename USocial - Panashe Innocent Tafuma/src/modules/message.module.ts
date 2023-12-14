import { Opt, Record } from "azle";
import { text } from "azle";
import { nat64 } from "azle";
import { Principal } from "azle";

export abstract class MessageConstants {
  public static readonly MAX_CONTENT_LENGTH = 800 as const;
  public static readonly MIN_CONTENT_LENGTH = 1 as const;
  public static readonly MAX_DELETE_MESSAGE_TOLERANCE = 3_600_000 as const; //1hr in milliseconds
}

export const Message = Record({
  id: text,
  createdAt: nat64,
  createdBy: Principal,
  userId: text,
  content: text,
  conversationId: text,
});

export type Message = typeof Message;

export const CreateMessageDto = Record({
  userId: text,
  content: text,
  conversationId: text,
});

export type CreateMessageDto = typeof CreateMessageDto;

export const UpdateMessageDto = Record({
  content: Opt(text),
  messageId: Opt(text),
});

export type UpdateMessageDto = typeof UpdateMessageDto;

export const PartialMessage = Record({
  content: text,
});

export type PartialMessage = typeof PartialMessage;
