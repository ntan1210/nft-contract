import { CDNTokenListResolutionStrategy } from "@solana/spl-token-registry";
import { formatNearAmount } from "near-api-js/lib/utils/format";
import { diffEpochValidators } from "near-api-js/lib/validators";
import React from "react"
import { useState, useEffect } from 'react'
import { async } from "regenerator-runtime";
import NFTCard from "./NFTCard";

export default function MyNFT() {
    const [listing, setListing] = useState([]);
    const [loaded, setLoaded] = useState(false);
    const [tokenData, setTokenData] = useState([])

    useEffect(() => {
        (async function get_listing() {
            let res = await window.contract.get_listing({ owner_id: window.accountId })
            setListing(res);
            setLoaded(true);
        })()
    }, [])

    useEffect(() => {
        listing.map(async (token_id) => {
            let res = await window.contract.get_token_data({ token_id });
            setTokenData(tokenData => [...tokenData, res])
        })
    }, [loaded == true])

    if (tokenData.length == listing.length) {
        tokenData.sort((a, b) => (a.token_id > b.token_id) ? 1 : -1);
        return (
            <div>
                {
                    <div className="nft-container">
                        {
                            tokenData.map((data) => {
                                return (
                                    <NFTCard key={data.token_id} data={data} />
                                    
                                )
                            })
                        }
                    </div>
                }
            </div>
        )
    }
    else 
        return (
            <h2>Loading...</h2>
        )
}