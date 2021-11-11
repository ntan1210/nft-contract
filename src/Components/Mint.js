import { Contract } from "near-api-js";
import { formatNearAmount, parseNearAmount } from "near-api-js/lib/utils/format";
import { diffEpochValidators } from "near-api-js/lib/validators";
import React from "react"
import {useState, useEffect} from 'react'
import { async } from "regenerator-runtime";


export default function Mint() {
    const [metadata, setMetadata] = useState("");

    async function call_mint_token() {
        let res = await window.contract.mint_token({
            args: {
                owner_id: window.accountId,
                metadata
            }, 
            amount: parseNearAmount("1"),
        })
        alert("Mint successfully");
    }

    return (
        <div>
            <form onSubmit={e => {
                e.preventDefault();
                call_mint_token();
            }}>
                <label>Mint your own NFT here:</label>
                <br />
                <input type="text"
                    value={metadata}
                    onChange={e => {setMetadata(e.target.value)}}
                    placeholder="enter the link"
                />
                <button type="submit">MINT</button>
            </form>
        </div>
    )
}