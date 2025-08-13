import { Metadata } from 'next'

import { Toaster } from '@/components/ui/sonner'

import '@/app/globals.css'

import { fontMono, fontMontserrat, fontSans } from '@/lib/fonts'
import { cn } from '@/lib/utils'
import { Providers } from '@/components/providers'
import { TailwindIndicator } from '@/components/tailwind-indicator'

import Main from './components/main'

export const metadata: Metadata = {
  title: {
    default: 'MSB CodeGen',
    template: `MSB CodeGen - %s`
  },
  description: 'MSB CodeGen, an AI coding assistant.',
  themeColor: [
    { media: '(prefers-color-scheme: light)', color: 'white' },
    { media: '(prefers-color-scheme: dark)', color: 'black' }
  ],
  icons: '/favicon.ico'
}

interface RootLayoutProps {
  children: React.ReactNode
}

export default function RootLayout({ children }: RootLayoutProps) {
  return (
    <html lang="en" suppressHydrationWarning>
      <head />
      <body
        className={cn(
          'bg-transparent font-sans antialiased',
          fontSans.variable,
          fontMono.variable,
          fontMontserrat.variable
        )}
      >
        <Providers attribute="class" defaultTheme="system" enableSystem>
          <Main>{children}</Main>
          <Toaster richColors closeButton />
          <TailwindIndicator />
        </Providers>
      </body>
    </html>
  )
}
