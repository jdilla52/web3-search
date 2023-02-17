# web3-search

An exploration into finding the creation block of a web3 contract via weighted binary search. This is a useful approach when you have a coorelation or where a loose heurstic can be applied to approximate the target value. In this case we often knew the aproximate age of a web3 contract based on indexed transactions.

The main idea is to provide a guess and bias the binary search towards it. As the search deepens the algorithm interpolates to traditional binary search. Here an addtional onshot mask is appied as we also knew of some top end transactions which the contract obviously had to predate.
