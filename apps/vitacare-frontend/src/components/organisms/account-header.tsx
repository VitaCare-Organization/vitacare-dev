export default function AccountHeader() {
    return (
      <div className="text-center space-y-2 mb-6">
        <div className="flex justify-center">
          <div className="w-10 h-10 rounded-full bg-blue-50 flex items-center justify-center">
            <svg
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
              className="text-primary"
            >
              <path
                d="M12 12C14.7614 12 17 9.76142 17 7C17 4.23858 14.7614 2 12 2C9.23858 2 7 4.23858 7 7C7 9.76142 9.23858 12 12 12Z"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              />
              <path
                d="M20 21C20 18.7909 18.7909 16.7909 17 15.5C15.2091 14.2091 13.2091 13.5 12 13.5C10.7909 13.5 8.79086 14.2091 7 15.5C5.20914 16.7909 4 18.7909 4 21"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              />
            </svg>
          </div>
        </div>
        <h1 className="text-2xl font-bold text-gray-900">Create an account</h1>
        <p className="text-sm text-gray-500">Register to access the VitaCare platform</p>
      </div>
    )
  }
  
  