#include <iostream>
#include <vector>
#include <fstream>
#include <algorithm>
using namespace std;

vector<int> read_input(const char *input_file);
int get_max_calories_index(const char *input_file);

int main(int argc, char const *argv[])
{
    cout << "Most calories: " << get_max_calories_index("input.txt") << endl;
    return 0;
}

int get_max_calories_index(const char *input_file)
{
    vector<int> numbers = read_input(input_file);
    int max_element_index = 0;
    for (int i = 1; i < numbers.size(); i++)
    {
        if (numbers[max_element_index] < numbers[i])
        {
            max_element_index = i;
        }
    }
    return numbers[max_element_index];
}

vector<int> read_input(const char *input_file)
{
    vector<int> return_vector;
    ifstream file_content(input_file);
    string line;
    int sum = -1;
    do
    {
        if (line == "" || sum == -1)
        {
            if (sum != -1)
            {
                return_vector.push_back(sum);
            }
            sum = 0;
        }
        else
        {
            sum += stoi(line);
        }
    } while (getline(file_content, line));

    return return_vector;
}