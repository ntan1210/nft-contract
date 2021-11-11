import { formatNearAmount } from "near-api-js/lib/utils/format";
import { diffEpochValidators } from "near-api-js/lib/validators";
import React from "react"
import {useState, useEffect} from 'react'
import { async } from "regenerator-runtime";
import Metadata from "./Metadata";
import Item from "./Item";

export default function Market() {
    const [marketListing, setMarketListing] = useState([])

    useEffect(() => {
        (async () => {
            let res = await window.contract.get_market_listing();
            setMarketListing(res);
        })()
    }, [])

    return (
        <div className="main">
            <div className="metadata">
                <Metadata />
            </div>
            <div className="market-listing">
                {
                    (!marketListing.length) ? 
                    (<h1>Loading the listing....</h1>) :
                    <div className="item-container">
                        {
                            marketListing.map((item) => {
                                return (<Item key={item.token_id} data={item} />)
                            })
                        }
                    </div>
                    
                }
            </div>
        </div>
    )
}