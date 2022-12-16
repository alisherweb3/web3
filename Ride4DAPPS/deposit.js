let txs = seeds.map(function(x){
  return invokeScript({
  dApp: dappAddress,
  call:{
    function:"deposit",
    args: []},
      payment: [{amount: 2*amount, asset:null }]
  }, x)
})
