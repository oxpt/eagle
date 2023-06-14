'use client';

import Footer from '@/components/visitors/footer';
import Header from '@/components/visitors/header';
import TopPage from '@/components/visitors/topPage';

export default function Home() {
  return (
    <>
      <Header />
      <main>
        <TopPage />
      </main>
      <Footer />
    </>
  );
}
