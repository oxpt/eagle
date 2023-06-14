import Footer from '@/components/visitors/footer';
import Header from '@/components/visitors/header';

export default function Layout({ children }) {
  return (
    <>
      <Header />
      <main className="min-h-[100svh]">{children}</main>
      <Footer />
    </>
  );
}
