import useSWR from "swr";
import { useState } from "react";
import { AccountService } from "@/domains/accounts/services/AccountService";
import { useAuth } from "../auth/useAuth";
import { Account } from "@/domains/accounts/entities/Account";

export function useAccounts(
  initialPage: number = 1,
  initialPageSize: number = 10
) {
  const { isAuthenticated } = useAuth();
  const [page, setPage] = useState(initialPage);
  const [pageSize, setPageSize] = useState(initialPageSize);

  const { data, error, isLoading, mutate } = useSWR(
    isAuthenticated ? `/accounts?page=${page}&page_size=${pageSize}` : null,
    () => AccountService.getAccounts({ page, page_size: pageSize })
  );

  async function updateAccount(id: number, new_username: string) {
    try {
      await AccountService.updateAccount(id, new_username);
      mutate((currentData: { total: number; data: Account[] } | undefined) => {
        if (!currentData) return currentData;
        return {
          total: currentData.total,
          data: currentData.data.map((account) =>
            account.getId() === id ? new Account(id, new_username) : account
          ),
        };
      }, false);
    } catch (error) {
      console.error("Failed to update account:", error);
      throw new Error("Failed to update account");
    }
  }

  return {
    accounts: data?.data,
    total: data?.total,
    isLoading,
    error,
    page,
    setPage,
    pageSize,
    setPageSize,
    updateAccount,
  };
}
