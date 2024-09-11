#ifndef BOARD_H
#define BOARD_H

#include <stdexcept>

class Pair {
    public:
        int x;
        int y;

        Pair(int x, int y) : x(x), y(y) {};
};

class Board {
public:
    int** board;
    int row;
    int col;

public:
    Board() {
        row = 0;
        col = 0;
        board = new int*[row];
        for (int i = 0; i < row; i++) {
            board[i] = new int[col];
        }
    }

    Board(int row, int col, int** board) {
        if (row != 15 || col != 15) {
            throw std::invalid_argument("Invalid board size");
        }

        this->row = row;
        this->col = col;
        this->board = board;
    }

    ~Board() {
        for (int i = 0; i < row; i++) {
            delete[] board[i];
        }
        delete[] board;
        }

    bool isValid(int x, int y) {
        return x >= 0 && x < row && y >= 0 && y < col;
    }
};

#endif