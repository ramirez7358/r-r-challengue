"use client";

import { useState, useCallback } from "react";
import SearchBar from "../components/SearchBar";
import AccountBalance from "../components/AccountBalance";
import TransactionList from "../components/TransactionList";
import { searchTransactions, getWalletBalance } from "../api/transactions.jsx";
import CreateTransactionButton from "../components/CreateTransactionButton";
import Toast from "../components/Toast";

function App() {
  const [searchQuery, setSearchQuery] = useState("");
  const [transactions, setTransactions] = useState([]);
  const [account, setAccount] = useState(null);
  const [walletBalance, setWalletBalance] = useState(null);
  const [isSearching, setIsSearching] = useState(false);
  const [toast, setToast] = useState(null);

  const handleSearch = async (query) => {
    if (!query.trim()) return;

    setSearchQuery(query);
    setIsSearching(true);
    try {
      const { transactions, account } = await searchTransactions(query);
      const balance = await getWalletBalance(query);
      setTransactions(transactions);
      setAccount(account);
      setWalletBalance(balance);
    } catch (error) {
      console.error("Error searching transactions:", error);
    } finally {
      setIsSearching(false);
    }
  };

  const handleClear = () => {
    setSearchQuery("");
    setTransactions([]);
    setAccount(null);
    setWalletBalance(null);
  };

  const refreshCurrentSearch = useCallback(async () => {
    if (searchQuery) {
      await handleSearch(searchQuery);
    }
  }, [searchQuery]);

  const showToast = (message, type = "success") => {
    setToast({ message, type });
  };

  const closeToast = () => {
    setToast(null);
  };

  return (
    <div className="container mx-auto px-4 py-8 max-w-5xl">
      {toast && (
        <Toast message={toast.message} type={toast.type} onClose={closeToast} />
      )}

      <div className="flex justify-between items-center mb-6">
        <h1 className="text-3xl font-bold">Transaction Search</h1>
        <CreateTransactionButton
          onTransactionCreated={() => {
            showToast("Transaction created successfully!");
            refreshCurrentSearch();
          }}
        />
      </div>

      <SearchBar
        onSearch={handleSearch}
        onClear={handleClear}
        isSearching={isSearching}
      />

      <div className="mt-6">
        {account && (
          <AccountBalance account={account} balance={walletBalance} />
        )}
      </div>

      <div className="mt-6">
        {transactions.length > 0 ? (
          <TransactionList transactions={transactions} />
        ) : (
          searchQuery &&
          !isSearching && (
            <div className="text-center py-10 bg-gray-50 rounded-lg">
              <p className="text-gray-500">
                No transactions found matching "{searchQuery}"
              </p>
            </div>
          )
        )}
      </div>
    </div>
  );
}

export default App;
