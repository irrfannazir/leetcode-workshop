#include <stdio.h>

// I programmed in leetcode from here
#include <stdlib.h>

int *twoSum(int* nums, int numSize, int target, int *returnSize){
    int *null_val = malloc(numSize * sizeof( int ));
    *returnSize = 2;
    
    for(int i = 0; i < numSize; i++){
        for(int j = i + 1; j < numSize; j++){
            if(nums[i] + nums[j] == target){
                int *result = malloc(2 * sizeof(int));
                result[0] = i;
                result[1] = j;
                return result;
            }
        }
    }
    
    *returnSize = 0;
    return NULL;
}

// To here
// Never mind the rest program

int main(){
    int ret;
    static const int target = 6;
    #define TEMP_SIZE 3
    static inline int arr[TEMP_SIZE] = {3,2,4};
    printf("%d %d\n", twoSum(arr, TEMP_SIZE, target, &ret)[0], twoSum(arr, TEMP_SIZE, target, &ret)[1]);
    #undef TEMP_SIZE
    return 0;
}