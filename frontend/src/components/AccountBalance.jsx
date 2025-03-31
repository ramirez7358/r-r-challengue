function AccountBalance({ account, balance }) {
  if (!account) return null;

  // Use the explicitly passed balance if available, otherwise use account.balance
  const displayBalance = balance !== null ? parseFloat(balance) : 0;

  return (
    <div className="bg-white p-6 rounded-lg shadow mb-6">
      <h2 className="text-lg font-semibold mb-4">Account Balance</h2>
      <div className="flex justify-between items-center">
        <div>
          <p className="text-sm text-gray-500">Account Address</p>
          <p className="font-medium">{account}</p>
        </div>
        <div className="text-right">
          <p className="text-sm text-gray-500">Current Balance</p>
          <p className="text-2xl font-bold">${displayBalance.toFixed(2)}</p>
        </div>
      </div>
    </div>
  );
}

export default AccountBalance;
