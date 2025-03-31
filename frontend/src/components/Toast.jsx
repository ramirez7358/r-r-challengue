"use client";

import { useEffect } from "react";
import { FiCheckCircle, FiAlertCircle, FiX } from "react-icons/fi";

function Toast({ message, type = "success", onClose, duration = 3000 }) {
  useEffect(() => {
    const timer = setTimeout(() => {
      onClose();
    }, duration);

    return () => clearTimeout(timer);
  }, [duration, onClose]);

  return (
    <div className="fixed top-4 right-4 z-50 animate-fade-in">
      <div
        className={`flex items-center p-4 rounded-lg shadow-lg ${
          type === "success"
            ? "bg-green-100 border-l-4 border-green-500"
            : type === "error"
            ? "bg-red-100 border-l-4 border-red-500"
            : "bg-blue-100 border-l-4 border-blue-500"
        }`}
      >
        <div className="flex-shrink-0 mr-3">
          {type === "success" ? (
            <FiCheckCircle className="h-5 w-5 text-green-500" />
          ) : (
            <FiAlertCircle className="h-5 w-5 text-red-500" />
          )}
        </div>
        <div className="flex-1 mr-2">
          <p
            className={`text-sm font-medium ${
              type === "success"
                ? "text-green-800"
                : type === "error"
                ? "text-red-800"
                : "text-blue-800"
            }`}
          >
            {message}
          </p>
        </div>
        <button
          onClick={onClose}
          className="flex-shrink-0 text-gray-400 hover:text-gray-500"
        >
          <FiX className="h-4 w-4" />
        </button>
      </div>
    </div>
  );
}

export default Toast;
