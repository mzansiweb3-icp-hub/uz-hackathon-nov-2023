import { AcceptFriendRequestDto, CreateFriendDto } from "../modules";
import validateUUID, { validateBoolean } from "./canister.util";

export function sanitiseCreateFriendDto(model: CreateFriendDto): void {
  validateUUID(model?.sourceId, model?.targetId);
}

export function sanitiseAcceptFriendRequestDto(
  model: AcceptFriendRequestDto
): void {
  validateUUID(model?.id);
  validateBoolean(model.isAccepted);
}
