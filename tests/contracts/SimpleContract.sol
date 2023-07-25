// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.15;

contract SimpleContract {
    uint256 a = 42;

    function f() public {
        a = a * 2;
    }
}
