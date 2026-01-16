export class Password {
  private constructor(private readonly value: string) {}

  static create(value: string): Password {
    if (!value) {
      throw new Error("Password cannot be empty");
    }
    if (value.length < 6 || value.length > 128) {
      throw new Error("Password must be between 6 and 128 characters");
    }
    return new Password(value.trim());
  }

  getValue(): string {
    return this.value;
  }

  equals(other: Password): boolean {
    if (other instanceof Password) {
      return this.value === other.value;
    }
    return false;
  }
}