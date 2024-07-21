'use client'

import React, { useState, useEffect } from 'react';
import { ContractIds } from '@/deployments/deployments'
import { zodResolver } from '@hookform/resolvers/zod'
import {
    contractQuery,
    decodeOutput,
    useInkathon,
    useRegisteredContract,
} from '@scio-labs/use-inkathon';

import { SubmitHandler, useForm } from 'react-hook-form'
import toast from 'react-hot-toast'
import * as z from 'zod'

import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Form, FormControl, FormItem, FormLabel } from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { contractTxWithToast } from '@/utils/contract-tx-with-toast'


interface DonationContractProps {
    // Define your props here if needed
    props?: any;
}
const formSchema = z.object({
    newMessage: z.string().min(1).max(90),
  })

export const DonationContract: React.FC<DonationContractProps> = () => {
    const { api, activeAccount, activeSigner } = useInkathon();
    const { contract, address: contractAddress } = useRegisteredContract(ContractIds.Donation);
    const [account, setAccount] = useState<string>('');
    const [donationAmount, setDonationAmount] = useState<number>(0);
    const [donationLoading, setDonationLoading] = useState<boolean>(false);
    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
      })
    const { register, reset, handleSubmit } = form;

    useEffect(() => {
        if (activeAccount) {
            setAccount(activeAccount.address);
        }
    }, [activeAccount]);

    const handleDonate = async () => {
        if (!activeAccount || !contract || !activeSigner || !api) {
            console.error('Wallet not connected. Try again...');
            return;
        }

        setDonationLoading(true);
        try {
            await contractTxWithToast(api, activeAccount.address, contract, 'donate', {}, [
                donationAmount,
            ]);
            console.log('Donation successful!');
        } catch (error) {
            console.error('Error occurred during donation:', error);
        } finally {
            setDonationLoading(false);
        }
    };

    return (
        <>
            <div className="flex max-w-[22rem] grow flex-col gap-4">
                <h2 className="text-center font-mono text-gray-400">Donation Contract</h2>
        <Form {...form}>

                <Card>
                    <CardContent className="pt-6">
                        <FormItem>
                            <FormLabel className="text-base">Account</FormLabel>
                            <FormControl>
                                <Input placeholder={account} disabled={true} />
                            </FormControl>
                        </FormItem>
                    </CardContent>
                </Card>

                <Card>
                    <CardContent className="pt-6">
                        <form className="flex flex-col justify-end gap-2">
                            <FormItem>
                                <FormLabel className="text-base">Donation Amount</FormLabel>
                                <FormControl>
                                    <div className="flex gap-2">
                                        <Input
                                            type="number"
                                            value={donationAmount}
                                            onChange={(e) => setDonationAmount(Number(e.target.value))}
                                            disabled={donationLoading}
                                        />
                                        <Button
                                            type="button"
                                            className="bg-primary font-bold"
                                            disabled={donationLoading}
                                            isLoading={donationLoading}
                                            onClick={handleDonate}
                                        >
                                            Donate
                                        </Button>
                                    </div>
                                </FormControl>
                            </FormItem>
                        </form>
                    </CardContent>
                </Card>
                </Form>

                <p className="text-center font-mono text-xs text-gray-600">
                    {contract ? contractAddress : 'Loading...'}
                </p>
            </div>
        </>
    );
};

export default DonationContract;