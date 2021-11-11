import { CDNTokenListResolutionStrategy } from "@solana/spl-token-registry";
import { formatNearAmount, parseNearAmount } from "near-api-js/lib/utils/format";
import { diffEpochValidators } from "near-api-js/lib/validators";
import React from "react"
import { useState, useEffect } from 'react'
import { async } from "regenerator-runtime";

export default function NFTCard(prop) {
    const [price, setPrice] = useState(0);

    async function set_price() {
        let nearPrice = parseNearAmount(price.toString());
        //TODO: price is currently not U128 to submit to contract
        let res = await window.contract.set_price({
            args: { token_id: prop.data.token_id, price: parseNearAmount(price.toString()) },
            amount: parseNearAmount("0.001")
        })
    }


    return (
        <div className="card">
            <img className="nft-image" width="100%" height="250px" src={prop.data.metadata} />
            <div className="details">
                <p>ID: {prop.data.token_id}</p>
                <p>Price: {formatNearAmount(prop.data.price)}</p>
                <form onSubmit={e => {
                    e.preventDefault();
                    set_price();
                }}
                >
                    <p>Set price (N): </p>
                    <input
                        value={price}
                        type="number"
                        onChange={e => setPrice(e.target.value)}
                        placeholder={0}
                    />
                    <button type="submit">Submit</button>
                </form>
            </div>
        </div>
    )
}