import { formatNearAmount, parseNearAmount } from "near-api-js/lib/utils/format";
import React from "react"
import {useState, useEffect} from 'react'
import { async } from "regenerator-runtime";

export default function Metadata() {
    const [data, setData] = useState({});
    const [userBalance, setUserBalance] = useState(0);

    useEffect(() => {
        (async function getData() {
            let res = await window.account.state();
            setData(res);
        })();
        ((async () => {
            let res = await window.contract.get_balance({ owner_id: window.accountId });
            setUserBalance(res);
        }))();
    }, [])

    async function withdraw() {
        let res = await window.contract.withdraw({
            args: {},
            amount: parseNearAmount("0.001")
        });
    }

    return (
        <div>
            <h5>Hello {window.accountId}</h5>
            <h5>Your balance is {formatNearAmount(data.amount, 3)} (N)</h5>
            <h5>Proceeding balance: {formatNearAmount(userBalance, 3)} (N)</h5>
            <form
                onSubmit={e => {
                    e.preventDefault();
                    withdraw();
                }}
            >
                <button>Withdraw</button>
            </form>
        </div>
    )
}