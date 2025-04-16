#define GLM_ENABLE_EXPERIMENTAL
// Core GLM functionality
#include <glm/glm.hpp>            // Basic types (vec2, vec3, mat4, etc.)
#include <glm/gtc/matrix_transform.hpp> // Common transformations (translate, rotate, perspective)
#include <glm/gtc/type_ptr.hpp>        // Conversion to/from pointer types
#include <glm/gtc/constants.hpp>       // Useful constants (pi, e, etc.)
#include <glm/gtx/transform.hpp>       // Extra transformation utilities
#include <glm/gtx/quaternion.hpp>      // Quaternion operations
#include <glm/gtx/euler_angles.hpp>    // Euler angle functions
#include <glm/gtx/matrix_decompose.hpp>// Matrix decomposition

// Optional for advanced math
#include <glm/gtx/norm.hpp>            // Fast length and distance calculations
#include <glm/gtx/string_cast.hpp>     // Convert glm types to strings (great for debugging)
#include <glm/ext.hpp>                 // Includes all GLM core + extensions

// And your original include
#include <glm/common.hpp>              // Common math functions
