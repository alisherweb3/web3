# web3 JS

From JS to Smart-Contracts. Modern React, web3 frameworks for decentralized applications. All learning will practice in fight project change with smart contracts

# JS

## Arrow functions

```javascript
const square = (number) => number * number;
```

## Parameters vs Arguments

Let's try replacing name, with firstName.
```javascript
const sayHi = (firstname) =? {
  console.log('Hi, ${firstName}');
};

sayHi('Joe'); // Hi Joe
```

Making a function call:
```javascript
const logAge = (name, age) => {
  console.log('${firstName} is ${age} years old.');
};

logAge('Joe', 25); // Joe is 25 years old.
```



## Global Scope

```
const name = 'Adrian';
```

## Local Scope

```
// Global Scope

const someFunction = () => {
  // Local Scope #1
  
  const anotherFunction = () => {
    // Local Scope #2
  };
};
```
