import { Contract } from "near-api-js";
import { formatNearAmount, parseNearAmount } from "near-api-js/lib/utils/format";
import { diffEpochValidators } from "near-api-js/lib/validators";
import React from "react"
import {useState, useEffect} from 'react'
import { async } from "regenerator-runtime";

export default function Item(prop) {

    async function purchase_token() {
        let res = await window.contract.purchase({
            args: {
                token_id: prop.data.token_id,
            },
            amount: prop.data.price
        })
    }

    return (
        <div className="item-card">
            <img className="nft-image" width="100%" height="250px" src={prop.data.metadata} />
            <div className="details">
                <p>ID: {prop.data.token_id}</p>
                <p>Price: {formatNearAmount(prop.data.price)}</p>
                <p>Owner: {prop.data.owner_id}</p>
                <form
                    onSubmit={e => {
                        e.preventDefault();
                        purchase_token();
                    }}
                >
                    <button>Buy</button>
                </form>
            </div>
        </div>
    )
}