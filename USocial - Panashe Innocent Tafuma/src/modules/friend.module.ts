import { Principal, Record, nat64 } from "azle";
import { text } from "azle";
import { bool } from "azle";

export const Friend = Record({
  sourceId: text,
  targetId: text,
  isAccepted: bool,
  id: text,
  createdAt: nat64,
  createdBy: Principal,
});

export type Friend = typeof Friend;

export const CreateFriendDto = Record({
  sourceId: text,
  targetId: text,
});

export type CreateFriendDto = typeof CreateFriendDto;

export const AcceptFriendRequestDto = Record({
  id: text,
  isAccepted: bool,
});

export type AcceptFriendRequestDto = typeof AcceptFriendRequestDto;
