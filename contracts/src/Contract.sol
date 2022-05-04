// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.10;

contract MyContract {
    struct Point {
        uint256 x;
        uint256 y;
    }

    struct Bundle {
        Point[] points;
        SubBundle[] sub;
    }

    struct SubBundle {
        uint256 id;
        Point point;
    }

    event NewPoint(Point x);
    event NewBundle(Bundle b);

    function submitPoint(Point memory _point) public {
        emit NewPoint(_point);
    }

    function submitBundle(Bundle memory _bundle) public {
        emit NewBundle(_bundle);
    }
}
