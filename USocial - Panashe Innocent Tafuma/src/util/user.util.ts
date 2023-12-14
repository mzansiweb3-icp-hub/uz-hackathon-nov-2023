import { Opt } from "azle";
import {
  CreateUserDto,
  PartialUser,
  UpdateUserDto,
  User,
  UserConstants,
} from "../modules";

export function validateUsername(username: string): void {
  if (
    username.length > UserConstants.MAX_USERNAME_LENGTH ||
    username.length < UserConstants.MIN_USERNAME_LENGTH
  ) {
    throw new Error(
      `Username must be between ${UserConstants.MIN_USERNAME_LENGTH} and ${UserConstants.MAX_USERNAME_LENGTH} characters long`
    );
  }
}

export function validateEmail(email: string): void {
  const EmailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

  if (!EmailRegex.test(email)) {
    throw new Error(`Email: ${email} is not valid`);
  }
}

export function validateCreateUserDto(
  model: CreateUserDto
): asserts model is CreateUserDto {
  if (typeof model.fullname !== "string" || !model.fullname.length) {
    throw new Error(`Invalid fullname: ${model.fullname}`);
  }

  if (typeof model.surname !== "string" || !model.surname.length) {
    throw new Error(`Invalid surname: ${model.surname}`);
  }

  if (typeof model.mobile !== "string" || !model.mobile.length) {
    throw new Error(`Invalid mobile: ${model.mobile}`);
  }

  if (typeof model.username !== "string" || !model.username.length) {
    throw new Error(`Invalid username: ${model.username}`);
  }

  validateUsername(model.username);

  if (typeof model.email !== "string" || !model.email.length) {
    throw new Error(`Invalid email: ${model.email}`);
  }

  validateEmail(model.email);
}

export function sanitiseUserUpdate(
  user: User,
  update: UpdateUserDto
): PartialUser {
  if (!user) {
    throw new Error("Missing user for update sanitisation");
  }

  if (!Object.values(update).length) {
    throw new Error("Empty user update payload");
  }

  let username = user.username;
  if (update?.username && typeof update?.username === "string") {
    validateUsername(update.username);
    username = update.username;
  }

  let email = user.email;
  if (update?.email && typeof update?.email === "string") {
    validateEmail(update.email);
    email = update.email;
  }

  let avatarURL = user?.avatarURL;

  let mobile = user.mobile;
  if (update?.mobile && typeof update?.mobile === "string") {
    mobile = update.mobile;
  }

  return {
    username,
    email,
    avatarURL,
    mobile,
  };
}
