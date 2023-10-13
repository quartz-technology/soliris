// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.15;

error BrokenQuartz();

enum QuartzType {
    Amethyst,
    Agate
}

struct Quartz {
    QuartzType variety;
}

type ShortString is bytes32;

contract SimpleContract {
    uint256 a = 42;

    event QuartzMined(QuartzType indexed variety);

    function IsAmethystBeautiful() public returns (bool) {
        return true;
    }
}
