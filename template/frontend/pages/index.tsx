import { useRef } from "react";
import { create_user } from "@functions";

const App = () => {
  const usernameRef = useRef<HTMLInputElement>(null);
  const emailRef = useRef<HTMLInputElement>(null);
  const passwordRef = useRef<HTMLInputElement>(null);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    const username = usernameRef.current?.value;
    const email = emailRef.current?.value;
    const password = passwordRef.current?.value;

    if (username && email && password) {
      create_user(email, password, username)
        .then((res) => {
          console.log("User created:", res);
        })
        .catch((err) => {
          console.error("Error creating user:", err);
        });
    }
  };

  return (
    <main className=" flex flex-row max-sm:flex-col font-sans w-full h-screen">
      <div className="flex flex-col w-1/2 max-sm:w-full text-center items-center justify-center bg-black">
        <h1 className="text-xl ">What is going on here??</h1>
        <br />
        <p className="w-full max-w-sm min-w-[200px]">
          When you click on the "create user" button, the typescript fuction
          "create_user()" calls a corresponding rust function also called
          "create_user()" found in the src/functions.rs and passes the
          parameters and the response all in their correct types. <br />
          What this means is that you can create rust functions in the
          src/functions.rs file and directly import them into your typescript
          app, all without doing any extra work of creating APIs or validating
          its data and return types. <br />
          It just works.
        </p>
      </div>
      <div className="flex flex-col items-center justify-center w-1/2 max-sm:w-full">
        <form
          onSubmit={handleSubmit}
          className="flex flex-col items-center justify-center w-full max-w-sm min-w-[200px] space-y-3"
        >
          <h6>Create A User</h6>
          <input
            type="text"
            name="username"
            ref={usernameRef}
            placeholder="Username"
            required
            className="w-full bg-transparent text-sm border border-slate-200 rounded-md px-3 py-2 transition duration-300 ease focus:outline-none focus:border-slate-400 hover:border-slate-300 shadow-sm focus:shadow"
          />
          <input
            type="email"
            name="email"
            ref={emailRef}
            placeholder="Email"
            required
            className="w-full bg-transparent text-sm border border-slate-200 rounded-md px-3 py-2 transition duration-300 ease focus:outline-none focus:border-slate-400 hover:border-slate-300 shadow-sm focus:shadow"
          />
          <input
            type="password"
            name="password"
            ref={passwordRef}
            placeholder="Password"
            required
            className="w-full bg-transparent text-sm border border-slate-200 rounded-md px-3 py-2 transition duration-300 ease focus:outline-none focus:border-slate-400 hover:border-slate-300 shadow-sm focus:shadow"
          />
          <button type="submit">Create User</button>
        </form>
      </div>
    </main>
  );
};

export default App;
