import Footer from "@/components/visitors/footer";
import Header from "@/components/visitors/header";
import { ReactNode } from "react";

interface Props {
  children: ReactNode;
}

export default function Layout({ children }: Props) {
  return (
    <div className="container mx-auto overflow-hidden px-1 sm:px-0">
      <Header />
      <main className="min-h-[calc(100svh_-_64px_-_264px)] sm:min-h-[calc(100svh_-_64px_-_144px)]">
        {children}
      </main>
      <Footer />
    </div>
  );
}
