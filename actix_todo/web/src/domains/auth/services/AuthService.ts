import { v4 as uuidv4 } from "uuid";

import { User } from "../entities/User";
import { Username } from "../value-objects/Username";
import { Password } from "../value-objects/Password";
import * as apis from "@/infrastructure/api/auth/api";
import { setToken } from "@/infrastructure/storage/tokenStorage";

export interface AuthResponse {
  token: string;
  user: {
    id: number;
    username: string;
  };
}

export class AuthService {
  static async login(username: string, password: string): Promise<User> {
    const usernameVO = Username.create(username);
    const passwordVO = Password.create(password);
    const response = await apis.login({
      username: usernameVO.getValue(),
      password: passwordVO.getValue(),
    });
    setToken(response.token);
    return new User(uuidv4(), usernameVO);
  }

  static async register(username: string, password: string): Promise<User> {
    const usernameVO = Username.create(username);
    const passwordVO = Password.create(password);
    await apis.registerAccount({
      username: usernameVO.getValue(),
      password: passwordVO.getValue(),
    });
    return new User(uuidv4(), usernameVO);
  }

  static async forgotPassword(
    username: string,
    newPassword: string
  ): Promise<void> {
    const usernameVO = Username.create(username);
    const passwordVO = Password.create(newPassword);
    await apis.resetPasswordByUsername(usernameVO.getValue(), {
      new_password: passwordVO.getValue(),
    });
  }
}
