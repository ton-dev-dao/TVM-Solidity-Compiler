/*
	This file is part of solidity.

	solidity is free software: you can redistribute it and/or modify
	it under the terms of the GNU General Public License as published by
	the Free Software Foundation, either version 3 of the License, or
	(at your option) any later version.

	solidity is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU General Public License for more details.

	You should have received a copy of the GNU General Public License
	along with solidity.  If not, see <http://www.gnu.org/licenses/>.
*/
// SPDX-License-Identifier: GPL-3.0
/**
 * @date 2017
 * Enums for AST classes.
 */

#pragma once

#include <liblangutil/Exceptions.h>
#include <libsolidity/ast/ASTForward.h>

#include <string>

namespace solidity::frontend
{

/// Possible lookups for function resolving
enum class VirtualLookup { Static, Virtual, Super };

// How a function can mutate the EVM state.
enum class StateMutability { Pure, View, NonPayable };

/// Visibility ordered from restricted to unrestricted.
enum class Visibility { Default, Private, Internal, Public, External, Getter };

enum class Arithmetic { Checked, Wrapping };

inline std::string stateMutabilityToString(StateMutability const& _stateMutability)
{
	switch (_stateMutability)
	{
	case StateMutability::Pure:
		return "pure";
	case StateMutability::View:
		return "view";
	case StateMutability::NonPayable:
		return "nonpayable";
	default:
		solAssert(false, "Unknown state mutability.");
	}
}

class Type;

/// Container for function call parameter types & names
struct FuncCallArguments
{
	/// Types of arguments
	std::vector<Type const*> types;
	/// Names of the arguments if given, otherwise unset
	std::vector<ASTPointer<ASTString>> names;
	/// it's for functions that have variable count of parameters or some params are skipped
	std::vector<Type const*> targetTypes;

	size_t numArguments() const { return types.size(); }
	size_t numNames() const { return names.size(); }
	bool hasNamedArguments() const { return !names.empty(); }
};

enum class ContractKind { Interface, Contract, Library };

}
