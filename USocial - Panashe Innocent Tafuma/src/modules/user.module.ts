import { Opt, Principal, Record, bool, nat64, text } from "azle";

export abstract class UserConstants {
  public static readonly MAX_USERNAME_LENGTH = 18 as const;
  public static readonly MIN_USERNAME_LENGTH = 3 as const;
}

export const User = Record({
  id: text,
  createdAt: nat64,
  createdBy: Principal,
  username: text,
  email: text,
  fullname: text,
  surname: text,
  avatarURL: Opt(text),
  isVerfied: bool,
  mobile: text,
});

export type User = typeof User;

export const CreateUserDto = Record({
  username: text,
  email: text,
  fullname: text,
  surname: text,
  avatarURL: Opt(text),
  mobile: text,
});

export type CreateUserDto = typeof CreateUserDto;

export const UpdateUserDto = Record({
  username: Opt(text),
  email: Opt(text),
  avatarURL: Opt(text),
  mobile: Opt(text),
});

export type UpdateUserDto = typeof UpdateUserDto;

export const PartialUser = Record({
  username: text,
  email: text,
  avatarURL: Opt(text),
  mobile: text,
});

export type PartialUser = typeof PartialUser;
