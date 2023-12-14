import { Record } from "azle";
import { text } from "azle";
import { nat64 } from "azle";
import { Principal } from "azle";

export const Conversation = Record({
  sourceId: text,
  targetId: text,
  id: text,
  createdAt: nat64,
  createdBy: Principal,
});

export type Conversation = typeof Conversation;

export const CreateConversationDto = Record({
  sourceId: text,
  targetId: text,
});

export type CreateConversationDto = typeof CreateConversationDto;
