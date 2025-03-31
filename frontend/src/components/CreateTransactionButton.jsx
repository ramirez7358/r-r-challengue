"use client";

import { useState } from "react";
import { FiPlus } from "react-icons/fi";
import CreateTransactionModal from "./CreateTransactionModal";

function CreateTransactionButton({ onTransactionCreated }) {
  const [isModalOpen, setIsModalOpen] = useState(false);

  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);

  const handleTransactionCreated = () => {
    closeModal();
    if (onTransactionCreated) {
      onTransactionCreated();
    }
  };

  return (
    <>
      <button
        onClick={openModal}
        className="flex items-center gap-2 px-4 py-2 bg-green-500 text-white rounded-md hover:bg-green-600 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2"
      >
        <FiPlus className="h-5 w-5" />
        Create Transaction
      </button>

      {isModalOpen && (
        <CreateTransactionModal
          onClose={closeModal}
          onSuccess={handleTransactionCreated}
        />
      )}
    </>
  );
}

export default CreateTransactionButton;
