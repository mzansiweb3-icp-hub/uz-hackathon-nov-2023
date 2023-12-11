import { Opt, Record, Vec, int32, nat64, text } from "azle";
import { Principal } from "azle";

export const Post = Record({
  id: text,
  createdAt: nat64,
  createdBy: Principal,
  userId: text,
  caption: text,
  tags: Opt(Vec(text)),
  likes: Vec(text),
});

export type Post = typeof Post;

export const CreatePostDto = Record({
  userId: text,
  caption: text,
  tags: Opt(Vec(text)),
});

export type CreatePostDto = typeof CreatePostDto;

export const UpdatePostDto = Record({
  caption: Opt(text),
  tags: Opt(Vec(text)),
});

export type UpdatePostDto = typeof UpdatePostDto;

export const PartialPost = Record({
  caption: text,
  tags: Opt(Vec(text)),
});

export type PartialPost = typeof PartialPost;
