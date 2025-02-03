import React from "react";

const AppLayout: React.FC<{ children?: React.ReactNode }> = ({ children }) => {
  return <main>{children}</main>;
};

export default AppLayout;
