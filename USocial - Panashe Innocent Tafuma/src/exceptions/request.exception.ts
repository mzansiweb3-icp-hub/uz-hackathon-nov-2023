import { Variant, text } from "azle";

export const RequestError = Variant({
  errMessage: text,
});

export type RequestError = typeof RequestError;
