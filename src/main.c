#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <time.h>
#include <argp.h>
#include <openssl/rand.h>

// Define constants using enums
const int MAX_PASSWORDS = 2048;
const int MIN_PASSWORDS = 1;
const int PASSWORD_LENGTH = 26;
const int LOWERCASE_LETTERS_COUNT = 18;
const int DIGIT_COUNT = 1;
const int UPPERCASE_LETTER_COUNT = 1;
const int DASH_POSITIONS[2]= {6, 13};

// Function to clamp a value within a given range
void clamp(int *value, int min, int max) {
	if (*value < min) {
		*value = min;
	} else if (*value > max) {
		*value = max;
	}
}

// Function to generate a random integer using OpenSSL's RAND_bytes
int get_random_int(void) {
	int random_integer;

	if (RAND_bytes((unsigned char*)&random_integer, sizeof(random_integer)) != 1) {
		perror("Error generating random bytes");
		exit(EXIT_FAILURE);
	}

	return abs(random_integer);
}

// Function to generate a random password
char *generate_password() {
	const char lowercase_letters[] = "abcdefghijklmnopqrstuvwxyz";
	char password[PASSWORD_LENGTH + 1];
	int random_index, i;

	// Generate lowercase letters
	for (i = 0; i < LOWERCASE_LETTERS_COUNT; i++) {
		random_index = get_random_int() % 26;
		password[i] = lowercase_letters[random_index];
	}

	// Generate a random digit
	password[LOWERCASE_LETTERS_COUNT] = '0' + get_random_int() % 10;

	// Generate a random uppercase letter
	password[LOWERCASE_LETTERS_COUNT + DIGIT_COUNT] = 'A' + get_random_int() % 26;

	// Shuffle the characters within the first 18 characters
	for (i = LOWERCASE_LETTERS_COUNT + DIGIT_COUNT; i > 0; i--) {
		random_index = get_random_int() % (i + 1);
		char temp = password[i];
		password[i] = password[random_index];
		password[random_index] = temp;
	}

	// Place dashes at specific positions
	for (i = 0; i < sizeof(DASH_POSITIONS) / sizeof(DASH_POSITIONS[0]); i++) {
		password[DASH_POSITIONS[i]] = '-';
	}

	// Null-terminate the string
	password[PASSWORD_LENGTH] = '\0';

	// Allocate memory for the password and copy the result
	char *result_password = (char *)calloc(PASSWORD_LENGTH + 1, sizeof(char));
	if (result_password != NULL) {
		strncpy(result_password, password, PASSWORD_LENGTH + 1);
	} else {
		perror("Error while generating the password");
		exit(EXIT_FAILURE);
	}

	return result_password;
}

const char *argp_program_version = "cpasswordgenerator 0.2.1";
const char *argp_program_bug_address = "<walker84837@gmail.com>";

static char doc[] = "Generate random passwords";
static char args_doc[] = "[OPTION]...";

struct arguments {
	int num_passwords;
};

static error_t parse_opt(int key, char *arg, struct argp_state *state)
{
	struct arguments *arguments = (struct arguments *)state->input;

	switch (key) {
		case 'n':
			arguments->num_passwords = atoi(arg);
			if (arguments->num_passwords <= 0) {
				fprintf(stderr, "Invalid number of passwords: %s\n", arg);
				exit(EXIT_FAILURE);
			}
			break;
		case ARGP_KEY_ARG:
			break;
		case ARGP_KEY_END:
			clamp(&arguments->num_passwords, MIN_PASSWORDS, MAX_PASSWORDS);
			break;
		default:
			return ARGP_ERR_UNKNOWN;
	}

	return 0;
}

static struct argp_option options[] = {
	{"num_passwords", 'n', "NUM_PASSWORDS", 0, "Number of passwords to generate", 0},
	{0}
};

static struct argp argp = {options, parse_opt, args_doc, doc};

int main(int argc, char *argv[])
{
	struct arguments args;
	args.num_passwords = 1;

	// Parse command-line arguments
	argp_parse(&argp, argc, argv, 0, 0, &args);

	// Generate and print passwords
	for (int i = 0; i < args.num_passwords; i++) {
		char *password = generate_password();
		if (password != NULL) {
			puts(password);
			free(password);
		} else {
			fputs("Failed to allocate memory for password.\n", stderr);
			return 1;
		}
	}

	return 0;
}
