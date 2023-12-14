export class UnauthorizedException extends Error {
  constructor(message: string = "Unauthorized") {
    super(message);

    Object.setPrototypeOf(this, UnauthorizedException.prototype);
    this.name = "UnauthorizedException";
  }
}
