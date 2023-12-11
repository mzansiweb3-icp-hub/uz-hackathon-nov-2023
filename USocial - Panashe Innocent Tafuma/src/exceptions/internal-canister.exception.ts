export class InternalCanisterError extends Error {
  constructor(message: string | undefined) {
    super(message);

    Object.setPrototypeOf(this, InternalCanisterError.prototype);
    this.name = "InternalCanisterError";
  }
}
