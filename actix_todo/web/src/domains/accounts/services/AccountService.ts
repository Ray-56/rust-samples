import { FetchAccountsRequest } from "@/infrastructure/api/accounts/types";
import * as apis from "@/infrastructure/api/accounts/api";
import { Account } from "../entities/Account";

export class AccountService {
  static async getAccounts(
    params: FetchAccountsRequest
  ): Promise<{ data: Account[]; total: number }> {
    const rsp = await apis.fetchAccounts(params);

    return {
      data: rsp.data.map(
        (account) => new Account(account.id, account.username)
      ),
      total: rsp.total,
    };
  }

  static async updateAccount(id: number, new_username: string): Promise<void> {
    await apis.updateAccount(id, { new_username });
  }
}
