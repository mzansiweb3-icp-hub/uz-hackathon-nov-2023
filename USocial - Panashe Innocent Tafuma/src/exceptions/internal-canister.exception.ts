export class InternalCanisterException extends Error {
  constructor(message: string | undefined) {
    super(message);

    Object.setPrototypeOf(this, InternalCanisterException.prototype);
    this.name = "InternalCanisterError";
  }
}
