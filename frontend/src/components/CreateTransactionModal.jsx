"use client";

import { useState } from "react";
import { FiX } from "react-icons/fi";
import { createTransaction } from "../api/transactions";

function CreateTransactionModal({ onClose, onSuccess }) {
  const [formData, setFormData] = useState({
    address_from: "",
    address_to: "",
    amount: "",
    transaction_type: "payment",
  });

  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState(null);

  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    setIsSubmitting(true);
    setError(null);

    try {
      await createTransaction(formData);

      // Call the success callback
      if (onSuccess) {
        onSuccess();
      }
    } catch (err) {
      setError(err.message || "Failed to create transaction");
      setIsSubmitting(false);
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg shadow-xl w-full max-w-md p-6 relative">
        <button
          onClick={onClose}
          className="absolute top-4 right-4 text-gray-500 hover:text-gray-700"
        >
          <FiX className="h-5 w-5" />
        </button>

        <h2 className="text-xl font-bold mb-4">Create New Transaction</h2>

        <form onSubmit={handleSubmit}>
          {error && (
            <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
              {error}
            </div>
          )}

          <div className="mb-4">
            <label className="block text-gray-700 text-sm font-bold mb-2">
              From Address
            </label>
            <input
              type="text"
              name="address_from"
              value={formData.address_from}
              onChange={handleChange}
              placeholder="0x04410Dad5053e8A9b1978016d86C58a058554C4E"
              className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              required
            />
          </div>

          <div className="mb-4">
            <label className="block text-gray-700 text-sm font-bold mb-2">
              To Address
            </label>
            <input
              type="text"
              name="address_to"
              value={formData.address_to}
              onChange={handleChange}
              placeholder="0x91a506CbB09436BFcAd535E156b07B96710d91D2"
              className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              required
            />
          </div>

          <div className="mb-4">
            <label className="block text-gray-700 text-sm font-bold mb-2">
              Amount
            </label>
            <input
              type="number"
              name="amount"
              value={formData.amount}
              onChange={handleChange}
              placeholder="100.1"
              step="0.01"
              min="0.01"
              className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              required
            />
          </div>

          <div className="mb-6">
            <label className="block text-gray-700 text-sm font-bold mb-2">
              Transaction Type
            </label>
            <select
              name="transaction_type"
              value={formData.transaction_type}
              onChange={handleChange}
              className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              required
            >
              <option value="withdrawal">Withdrawal</option>
              <option value="deposit">Deposit</option>
            </select>
          </div>

          <div className="flex justify-end gap-2">
            <button
              type="button"
              onClick={onClose}
              className="px-4 py-2 border border-gray-300 rounded-md text-gray-700 hover:bg-gray-50"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={isSubmitting}
              className="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
            >
              {isSubmitting ? "Creating..." : "Create Transaction"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

export default CreateTransactionModal;
