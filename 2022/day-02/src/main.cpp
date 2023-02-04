#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>
#include "Round.hpp"

std::vector<Round> readPuzzle(const char *fileName)
{
    std::vector<Round> retVector;
    std::string myText;

    std::ifstream MyReadFile(fileName);
    while (std::getline(MyReadFile, myText))
    {
        std::string data1 = myText.substr(0, myText.find(' '));

        Shape opponentShape;
        if (data1 == "A")
        {
            opponentShape = Shape::Rock;
        }
        else if (data1 == "B")
        {
            opponentShape = Shape::Paper;
        }
        else
        {
            opponentShape = Shape::Scissor;
        }

        std::string data2 = myText.substr(myText.find(' ') + 1, myText.length());

        Shape ownShape;
        if (data2 == "Y")
        {
            ownShape = Shape::Paper;
        }
        else if (data2 == "X")
        {
            ownShape = Shape::Rock;
        }
        else
        {
            ownShape = Shape::Scissor;
        }

        retVector.push_back(Round(ownShape, opponentShape));
    }

    MyReadFile.close();
    return retVector;
}

int main(int argc, char const *argv[])
{
    std::vector<Round> Game = readPuzzle("puzzle.txt");
    int pointCOunter = 0;
    for (int i = 0; i < Game.size(); i++)
    {
        pointCOunter += Game[i].CompareShapes();
        pointCOunter += Game[i].GetOwnShape();
    }

    std::cout << pointCOunter << "\n";

    return 0;
}