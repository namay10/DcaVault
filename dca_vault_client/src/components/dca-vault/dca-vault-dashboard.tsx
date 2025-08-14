'use client'

import { useState } from 'react'
import { useWalletUi } from '@wallet-ui/react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Badge } from '@/components/ui/badge'
import { Separator } from '@/components/ui/separator'
import { Clock, DollarSign, TrendingUp, Wallet, AlertCircle } from 'lucide-react'

interface VaultState {
  owner: string
  totalAmount: number
  periods: number
  intervalSeconds: number
  stakedAt: number
  currBalance: number
  periodsCompleted: number
  nextSwapTime: number
  totalSolReceived: number
  bump: number
}

export function DcaVaultDashboard() {
  const { account } = useWalletUi()
  const connected = !!account
  const publicKey = account?.address
  const [vaultState, setVaultState] = useState<VaultState | null>(null)
  const [loading, setLoading] = useState(false)

  // Form state for vault initialization
  const [amount, setAmount] = useState('')
  const [periods, setPeriods] = useState('')
  const [interval, setInterval] = useState('')

  const handleInitializeVault = async () => {
    if (!connected || !publicKey) return

    setLoading(true)
    try {
      // TODO: Implement vault initialization
      console.log('Initializing vault with:', { amount, periods, interval })

      // Mock vault state for now
      setVaultState({
        owner: publicKey.toString(),
        totalAmount: parseFloat(amount) * 1000000, // Convert to USDC decimals
        periods: parseInt(periods),
        intervalSeconds: parseInt(interval),
        stakedAt: Date.now() / 1000,
        currBalance: parseFloat(amount) * 1000000,
        periodsCompleted: 0,
        nextSwapTime: Date.now() / 1000 + parseInt(interval),
        totalSolReceived: 0,
        bump: 0,
      })
    } catch (error) {
      console.error('Failed to initialize vault:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleSwap = async () => {
    if (!connected || !vaultState) return

    setLoading(true)
    try {
      // TODO: Implement swap functionality
      console.log('Executing swap')

      // Mock swap update
      setVaultState((prev) =>
        prev
          ? {
              ...prev,
              periodsCompleted: prev.periodsCompleted + 1,
              currBalance: prev.currBalance - prev.totalAmount / prev.periods,
              nextSwapTime: prev.nextSwapTime + prev.intervalSeconds,
              totalSolReceived: prev.totalSolReceived + 0.01, // Mock SOL received
            }
          : null,
      )
    } catch (error) {
      console.error('Failed to execute swap:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleWithdraw = async () => {
    if (!connected || !vaultState) return

    setLoading(true)
    try {
      // TODO: Implement withdraw functionality
      console.log('Withdrawing from vault')
      setVaultState(null)
    } catch (error) {
      console.error('Failed to withdraw:', error)
    } finally {
      setLoading(false)
    }
  }

  if (!connected) {
    return (
      <div className="flex flex-col items-center justify-center min-h-[400px] space-y-4">
        <Wallet className="h-12 w-12 text-muted-foreground" />
        <h2 className="text-2xl font-semibold">Connect Your Wallet</h2>
        <p className="text-muted-foreground text-center max-w-md">
          Connect your Solana wallet to start using the DCA Vault
        </p>
      </div>
    )
  }

  return (
    <div className="max-w-4xl mx-auto p-6 space-y-6">
      <div className="text-center space-y-2">
        <h1 className="text-3xl font-bold">DCA Vault</h1>
        <p className="text-muted-foreground">Dollar Cost Average into SOL automatically with your USDC</p>
      </div>

      {!vaultState ? (
        <Card>
          <CardHeader>
            <CardTitle>Initialize DCA Vault</CardTitle>
            <CardDescription>Set up your automated DCA strategy by depositing USDC</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div className="space-y-2">
                <Label htmlFor="amount">USDC Amount</Label>
                <Input
                  id="amount"
                  type="number"
                  placeholder="100"
                  value={amount}
                  onChange={(e) => setAmount(e.target.value)}
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="periods">Number of Periods</Label>
                <Input
                  id="periods"
                  type="number"
                  placeholder="10"
                  value={periods}
                  onChange={(e) => setPeriods(e.target.value)}
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="interval">Interval (seconds)</Label>
                <Select value={interval} onValueChange={setInterval}>
                  <SelectTrigger>
                    <SelectValue placeholder="Select interval" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="60">1 minute</SelectItem>
                    <SelectItem value="300">5 minutes</SelectItem>
                    <SelectItem value="3600">1 hour</SelectItem>
                    <SelectItem value="86400">1 day</SelectItem>
                  </SelectContent>
                </Select>
              </div>
            </div>
            <Button
              onClick={handleInitializeVault}
              disabled={loading || !amount || !periods || !interval}
              className="w-full"
            >
              {loading ? 'Initializing...' : 'Initialize Vault'}
            </Button>
          </CardContent>
        </Card>
      ) : (
        <div className="space-y-6">
          {/* Vault Status */}
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <TrendingUp className="h-5 w-5" />
                Vault Status
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <div className="text-center">
                  <p className="text-sm text-muted-foreground">Total Amount</p>
                  <p className="text-lg font-semibold">${(vaultState.totalAmount / 1000000).toFixed(2)} USDC</p>
                </div>
                <div className="text-center">
                  <p className="text-sm text-muted-foreground">Remaining Balance</p>
                  <p className="text-lg font-semibold">${(vaultState.currBalance / 1000000).toFixed(2)} USDC</p>
                </div>
                <div className="text-center">
                  <p className="text-sm text-muted-foreground">Periods Completed</p>
                  <p className="text-lg font-semibold">
                    {vaultState.periodsCompleted} / {vaultState.periods}
                  </p>
                </div>
                <div className="text-center">
                  <p className="text-sm text-muted-foreground">SOL Received</p>
                  <p className="text-lg font-semibold">{vaultState.totalSolReceived.toFixed(4)} SOL</p>
                </div>
              </div>
            </CardContent>
          </Card>

          {/* Next Swap Info */}
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Clock className="h-5 w-5" />
                Next Swap
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-muted-foreground">Next swap time</p>
                  <p className="text-lg font-semibold">{new Date(vaultState.nextSwapTime * 1000).toLocaleString()}</p>
                </div>
                <Badge variant={vaultState.nextSwapTime <= Date.now() / 1000 ? 'default' : 'secondary'}>
                  {vaultState.nextSwapTime <= Date.now() / 1000 ? 'Ready' : 'Pending'}
                </Badge>
              </div>
            </CardContent>
          </Card>

          {/* Actions */}
          <div className="flex gap-4">
            <Button
              onClick={handleSwap}
              disabled={
                loading ||
                vaultState.nextSwapTime > Date.now() / 1000 ||
                vaultState.periodsCompleted >= vaultState.periods
              }
              className="flex-1"
            >
              {loading ? 'Processing...' : 'Execute Swap'}
            </Button>
            <Button onClick={handleWithdraw} disabled={loading} variant="outline" className="flex-1">
              {loading ? 'Processing...' : 'Withdraw'}
            </Button>
          </div>

          {/* Warning */}
          {vaultState.periodsCompleted < vaultState.periods && (
            <Card className="border-yellow-200 bg-yellow-50 dark:border-yellow-800 dark:bg-yellow-950">
              <CardContent className="pt-6">
                <div className="flex items-center gap-2 text-yellow-800 dark:text-yellow-200">
                  <AlertCircle className="h-4 w-4" />
                  <p className="text-sm">Early withdrawal will incur a 0.5% fee on remaining balance</p>
                </div>
              </CardContent>
            </Card>
          )}
        </div>
      )}
    </div>
  )
}
