#include <stdio.h>
#include <stdlib.h>

int qs(int*, int, int);
int part(int*, int, int);
int print(int*, int);

int main()	{
	//generate an array to sort
	int arr[5];
	int* ap = arr;
	int size = sizeof(arr) / sizeof(arr[0]);
	for (int i = 0; i < size; i++)	{
		ap[i] = rand();
	}
	//print the array
	print(ap, size);
	//sort the array
	printf("Sorting..\n");
	qs(ap, 0, 4);
	print(ap, size);
	return 0;
}

int part(int* ap, int lo, int hi)	{
	int pivot = hi;
	int swp;
	for(int i = 0; i < hi; i++)	{
		if (ap[i] > ap[pivot])	{
			int swp = ap[i];
			ap[i] = ap[pivot];
			ap[pivot] = swp;
		}
	}
	return pivot;
}

int qs(int* ap, int lo, int hi)	{
	if(lo < hi)	{
	int pivot = part(ap, lo, hi);
	//pivot is now in the right place
	qs(ap, lo, pivot-1);
	qs(ap, pivot+1, hi);
	}
	return 0;
}

int print(int* ap, int size)	{
	printf("arr: [");
	for (int i = 0; i < size; i++)	{
		printf("%d, ", ap[i]);
	}
	printf("]\n");
	return 0;
}
