/**
 * Searches for transactions by calling the external API
 * @param {string} query - The search query (address)
 * @returns {Promise<{transactions: Array, account: Object|null}>}
 */
export async function searchTransactions(query) {
  if (!query) return { transactions: [], account: null };

  try {
    let uri = `http://127.0.0.1:8080/api/transactions/${query}`;
    const response = await fetch(uri);

    // Check if the request was successful
    if (!response.ok) {
      throw new Error(`API error: ${response.status} ${response.statusText}`);
    }

    // Parse the JSON response
    const data = await response.json();
    console.log("Data:", data);

    const transactions = data || [];

    // Extract account information if available
    // Assuming the API might return account info along with transactions
    const account = query || null;

    return { transactions, account };
  } catch (error) {
    console.error("Error fetching transactions:", error);
    // Return empty results in case of error
    return { transactions: [], account: null };
  }
}

/**
 * Fetches wallet balance for a specific address
 * @param {string} address - The wallet address
 * @returns {Promise<number|null>} - The wallet balance or null if error
 */
export async function getWalletBalance(address) {
  if (!address) return null;

  try {
    // Call the balance API endpoint
    const response = await fetch(
      `http://127.0.0.1:8080/api/wallet/balance/${encodeURIComponent(address)}`
    );

    // Check if the request was successful
    if (!response.ok) {
      throw new Error(`API error: ${response.status} ${response.statusText}`);
    }

    // Parse the JSON response
    const data = await response.json();

    // Return the balance from the response
    // Assuming the API returns an object with a balance property
    return data || 0;
  } catch (error) {
    console.error("Error fetching wallet balance:", error);
    return null;
  }
}

/**
 * Creates a new transaction
 * @param {Object} transactionData - The transaction data
 * @returns {Promise<Object>} - The created transaction
 */
export async function createTransaction(transactionData) {
  try {
    const response = await fetch("http://127.0.0.1:8080/api/transactions", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(transactionData),
    });

    if (!response.ok) {
      const contentType = response.headers.get("Content-Type");
      console.log(contentType);
      const errorData = await response.json();
      throw new Error(
        errorData.message ||
          `API error: ${response.status} ${response.statusText}`
      );
    }

    return await response.json();
  } catch (error) {
    console.error("Error creating transaction:", error);
    throw error;
  }
}
