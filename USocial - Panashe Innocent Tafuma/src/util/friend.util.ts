import { AcceptFriendRequestDto, CreateFriendDto } from "../modules";
import validateUUIDv4, { validateBoolean } from "./canister.util";

export function sanitiseCreateFriendDto(model: CreateFriendDto): void {
  validateUUIDv4(model?.sourceId);
  validateUUIDv4(model?.targetId);
}

export function sanitiseAcceptFriendRequestDto(
  model: AcceptFriendRequestDto
): void {
  validateUUIDv4(model?.id);
  validateBoolean(model.isAccepted);
}
