import '@/app/globals.css';
import localFont from 'next/font/local';
import Head from 'next/head';
import React from 'react';

// Font files can be colocated inside of `app`
const gothicFont = localFont({
  src: '../fonts/BIZUDPGothic-Regular.ttf',
  display: 'swap',
  variable: '--font-gothic',
});

const minchoFont = localFont({
  src: '../fonts/BIZUDPMincho-Regular.ttf',
  display: 'swap',
  variable: '--font-mincho',
});

const gothicFontBold = localFont({
  src: '../fonts/BIZUDPGothic-Bold.ttf',
  display: 'swap',
  variable: '--font-gothic-bold',
});

const minchoFontBold = localFont({
  src: '../fonts/BIZUDPMincho-Bold.ttf',
  display: 'swap',
  variable: '--font-mincho-bold',
});

export const metadata = {
  title: 'XEE.JP',
  description: 'XEE.JP operated by Eagle.',
  charSet: 'utf-8',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html
      lang="ja"
      className={`${gothicFont.variable} ${minchoFont.variable}  ${gothicFontBold.variable}  ${minchoFontBold.variable}`}
    >
      <Head>
        <meta name="viewport" content="width=device-width,initial-scale=1,user-scalable=0" />
        <link rel="icon" href="/favicon.ico" sizes="any" />
      </Head>
      <body>{children}</body>
    </html>
  );
}
